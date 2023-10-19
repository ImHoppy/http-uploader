#include <fstream>
#include <iostream>
#include <string>
#include "libs/httplib.h"

namespace fb
{
	unsigned int write_file(const std::string &file_path, const std::string &content)
	{
		std::ofstream ofs(file_path, std::ios::binary);
		if (!ofs.is_open())
		{
			return EXIT_FAILURE;
		}
		ofs.write(content.c_str(), content.length());
		ofs.close();
		return EXIT_SUCCESS;
	}

	unsigned int send_file(const std::string &file_path, httplib::Response &res, const std::string &mimetype)
	{
		std::ifstream ifs(file_path, std::ios::binary | std::ios::ate);
		if (!ifs.is_open())
		{
			res.status = 404;
            res.set_content("File not found", "text/plain");
			return EXIT_FAILURE;
		}
		std::ifstream::pos_type file_size = ifs.tellg();
		ifs.seekg(0, std::ios::beg);
		std::vector<char> file_data(file_size);
		ifs.read(file_data.data(), file_size);
		ifs.close();

		res.set_content(file_data.data(), file_size, mimetype);
		return EXIT_SUCCESS;
	}

} // namespace fb
