#ifndef _LIB_DEMO_FILE_H_2022_11_11
#define _LIB_DEMO_FILE_H_2022_11_11
#include <cstdint>
struct DemoFile;
struct Foo;

extern "C"
{
    //简单的加法
    uint32_t add(uint32_t left, uint32_t right);

    //打开文件
    int open_file(const char *filename, void **item_ptr);

    //读取文件内容
    int read_file_as_string(const DemoFile *file_ptr, char **item_ptr);

    //释放文件资源
    int free_file(DemoFile *file_ptr);

    //释放字符串资源
    int free_string(char *string_ptr);

    Foo *foo_new1();
    Foo *foo_new2();
    void foo_delete(Foo *f);
}
#endif