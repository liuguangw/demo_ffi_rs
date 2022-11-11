#include <iostream>
#include "lib_demo_file.h"
using std::cout;
using std::endl;

int main(int argc, char **argv)
{
    uint32_t left = 50;
    uint32_t right = 80;
    cout << left << " + " << right << " = " << add(left, right) << endl;
    //
    const char *filename;
    if (argc > 1)
    {
        filename = argv[1];
    }
    else
    {
        //如果没有传入参数
        filename = "../Cargo.toml";
    }
    //
    void *file_ptr = nullptr;
    if (open_file(filename, &file_ptr) != 0)
    {
        //打开文件失败
        char *error_message = (char *)file_ptr;
        cout << "open " << filename << " failed: " << error_message << endl;
        //释放由rust分配的字符串内存
        free_string(error_message);
        return -1;
    }
    char *file_content = nullptr;
    int ret = read_file_as_string(file_ptr, &file_content);
    if (ret != 0)
    {
        //读取文件内容失败
        cout << "read " << filename << " content failed: " << file_content << endl;
    }
    else
    {
        cout << "=====================" << endl
             << file_content
             << "=====================" << endl;
    }
    //释放由rust分配的字符串内存
    free_string(file_content);
    //释放由rust分配的File对象内存
    free_file(file_ptr);
    return ret;
}