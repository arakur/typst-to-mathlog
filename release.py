import os
import shutil

PLATFORM = "windows"

shutil.rmtree("./release")
os.mkdir("./release")
os.mkdir("./release/bin")

shutil.copy(
    "./target/release/typst-to-mathlog.exe", "./release/bin/typst-to-mathlog.exe"
)
shutil.copytree("./style", "./release/style")
shutil.copytree("./dictionary", "./release/dictionary")
shutil.copy("./README.md", "./release/README.md")

shutil.make_archive("./typst-to-mathlog-" + PLATFORM, "zip", "./release")
