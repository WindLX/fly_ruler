import os

def list_files_in_directory(folder_path):
    try:
        # 获取指定文件夹下的所有文件名
        files = os.listdir(folder_path)

        # 打印所有文件名
        print(f"Files in {folder_path}:")
        for file in files:
            print(f"\"{file}\",")
    except FileNotFoundError:
        print(f"Error: The specified folder '{folder_path}' does not exist.")
    except Exception as e:
        print(f"An error occurred: {e}")

# 指定文件夹路径
folder_path = "."

# 调用函数来列出文件夹下的所有文件名
list_files_in_directory(folder_path)
