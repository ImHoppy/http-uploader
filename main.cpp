#include <fstream>
#include <iostream>
#include <string>
#include <sys/stat.h>
#include <sys/types.h>
#include <thread>
#include <vector>
#include <map>
#include "libs/uuid_v4.h"
#include "libs/httplib.h"

#include "fb_cdn.h"

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
		if (!req.has_file("file"))
		{
			res.status = 400;
			res.set_content("No file uploaded.", "text/plain");
			return;
		}
		auto file = req.get_file_value("file");
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
		std::cout << "Retrive: " << file_name << std::endl;

        std::string file_path = upload_directory + file_name;

       fb::send_file(file_path, res, httplib::detail::find_content_type(file_name, mimetype_addons, "application/octet-stream"));
	});

	std::cout << "Server started at localhost:8080" << std::endl;
	svr.listen("localhost", 8080);
	return 0;
}
