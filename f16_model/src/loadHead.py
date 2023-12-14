import re

def extract_function_signatures(file_content):
    # 使用正则表达式提取函数定义及参数
    pattern = re.compile(r'(\w+)\s+(\**\w+)\s*\(([^)]*)\)\s*{')
    matches = pattern.findall(file_content)
    return matches

def generate_header_file(function_signatures):
    header_content = "#ifndef MY_HEADER_FILE_H\n#define MY_HEADER_FILE_H\n\n"

    for return_type, function_name, parameters in function_signatures:
        header_content += f"{return_type} {function_name}({parameters});\n"

    header_content += "\n#endif // MY_HEADER_FILE_H\n"

    return header_content

def main():
    # 读取C文件内容
    with open('mexndinterp.c', 'r') as file:
        c_file_content = file.read()

    # 提取函数签名
    function_signatures = extract_function_signatures(c_file_content)

    # 生成头文件内容
    header_content = generate_header_file(function_signatures)

    # 写入头文件
    with open('generated_header_file.h', 'w') as header_file:
        header_file.write(header_content)

if __name__ == "__main__":
    main()

