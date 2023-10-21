#include <fstream>
#include <iostream>
#include <string>
#include <sys/stat.h>
#include <sys/types.h>
#include <thread>
#include <vector>
#include <map>
#include <utime.h>

#include "libs/uuid_v4.h"
#include "libs/httplib.h"

#include "fb_cdn.h"

const unsigned int MAX_UPLOAD_SIZE = 1024 * 1024 * 1024; // 1 gigabyte (GB)
const unsigned int MAX_AGES = 60 * 60 * 24 * 7; // 1 week

int main()
{
	const std::string upload_directory = "./uploads/";
	if (!httplib::detail::is_dir(upload_directory.c_str()))
	{
		if (mkdir(upload_directory.c_str(), 0777) == -1)
		{
			std::cout << "Error :  " << strerror(errno) << std::endl;
			return EXIT_FAILURE;
		}
	}

	httplib::Server svr;
	UUIDv4::UUIDGenerator<std::mt19937_64> uuidGenerator;

	std::map<std::string, std::string> mimetype_addons;
	mimetype_addons["ogg"] = "audio/ogg";

	svr.Post("/upload", [&](const httplib::Request &req, httplib::Response &res)
	{
		if (!req.files.size())
		{
			res.status = 400;
			res.set_content("No file uploaded.", "text/plain");
			return;
		}
		auto file = req.files.begin()->second;
		if (file.content.length() > MAX_UPLOAD_SIZE)
		{
			res.status = 400;
			res.set_content("File is too large.", "text/plain");
			return;
		}
		const std::string &filename = file.filename;

		std::string uuid = uuidGenerator.getUUID().str();
		std::string new_filename = uuid + '.' + httplib::detail::file_extension(filename);
		std::string full_path = upload_directory + new_filename;
		if (fb::write_file(full_path, file.content) == EXIT_SUCCESS)
		{
			res.set_content(new_filename, "text/plain");
			std::cout << "Upload: " << filename << " " << new_filename << std::endl;
		}
		else
		{
			res.status = 500;
			res.set_content("Failed to save the uploaded file.", "text/plain");
		}
	});

	svr.Get("/file/:uuid", [&](const httplib::Request &req, httplib::Response &res)
	{
        std::string file_name = req.path_params.at("uuid");

        std::string file_path = upload_directory + file_name;

		const long now = time(nullptr);
		const long last_accessed = fb::get_file_last_accessed(file_path);
		std::cout << now << " - " << "Fetching: " << file_name << ", idling for " << now - last_accessed << " seconds" << std::endl;
		if (last_accessed == 0 || (now - last_accessed) > MAX_AGES)
		{
			res.status = 404;
			res.set_content("File not found", "text/plain");
			return;
		}
		if (now - last_accessed > 60)
		{
			const struct utimbuf a{now, now};
			utime(file_path.c_str(), &a);
		}

		fb::send_file(file_path, res, httplib::detail::find_content_type(file_name, mimetype_addons, "application/octet-stream"));

	});

	std::cout << "Server started at localhost:8080" << std::endl;
	svr.listen("0.0.0.0", 8080);
	return 0;
}
