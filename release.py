import shutil

shutil.rmtree("./release")

shutil.copytree("./bin", "./release/bin")
shutil.copytree("./style", "./release/style")
shutil.copytree("./dictionary", "./release/dictionary")
shutil.copy("./README.md", "./release/README.md")

# zip ./release
shutil.make_archive("./release", "zip", "./release")
