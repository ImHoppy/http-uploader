#include <fstream>
#include <iostream>
#include <string>
#include <sys/stat.h>
#include <thread>
#include <vector>
#include <map>
#include "libs/uuid_v4.h"
#include "libs/httplib.h"

int main()
{
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
		const std::string upload_directory = "./uploads/";

		// if (!httplib::detail::is_dir(upload_directory.c_str()))
		// {
		// 	if (!httplib::detail::(upload_directory.c_str()))
		// 	{
		// 		res.status = 500;
		// 		res.set_content("Failed to create the upload directory.", "text/plain");
		// 		return;
		// 	}
		// }

		std::string uuid = uuidGenerator.getUUID().str();
		std::string new_filename = uuid + '.' + httplib::detail::file_extension(filename);
		std::string full_path = upload_directory + new_filename;
		try
		{
			std::ofstream ofs(full_path, std::ios::binary);
			if (!ofs.is_open())
			{
				res.status = 500;
				res.set_content("Failed to save the uploaded file.", "text/plain");
				return;
			}
			ofs.write(file.content.c_str(), file.content.length());
			ofs.close();
            res.set_content(new_filename, "text/plain");
			std::cout << "Upload: " << filename << " " << new_filename << std::endl;
		}
		catch(const std::exception& e)
		{
			std::cerr << e.what() << '\n';
			res.status = 500;
			res.set_content("Failed to save the uploaded file.", "text/plain");
		} });

	svr.Get("/file/:uuid", [&](const httplib::Request &req, httplib::Response &res)
			{
        std::string file_name = req.path_params.at("uuid");
		std::cout << "Retrive: " << file_name << std::endl;

        const std::string upload_directory = "./uploads/";
        std::string file_path = upload_directory + file_name;

		std::ifstream file_stream(file_path, std::ios::binary | std::ios::ate);
        if (file_stream) {
            std::ifstream::pos_type file_size = file_stream.tellg();
            file_stream.seekg(0, std::ios::beg);
            std::vector<char> file_data(file_size);
            file_stream.read(file_data.data(), file_size);

            res.set_content(file_data.data(), file_size, httplib::detail::find_content_type(file_name, mimetype_addons, "application/octet-stream"));
        } else {
            res.status = 404;
            res.set_content("File not found", "text/plain");
        } });

	std::cout << "Server started at localhost:8080" << std::endl;
	svr.listen("localhost", 8080);
	return 0;
}
