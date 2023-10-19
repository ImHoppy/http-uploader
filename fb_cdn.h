#pragma once
#include <string>
#include "libs/httplib.h"

namespace fb
{
	unsigned int write_file(const std::string &file_path, const std::string &content);
	unsigned int send_file(const std::string &file_path, httplib::Response &res, const std::string &mimetype);
	time_t get_file_last_accessed(const std::string &file_path);
}
