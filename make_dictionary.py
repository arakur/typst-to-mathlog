from typing import Any
from typing import Type
import json


class ModDict:
    idents: dict[str, str]
    modules: dict[str, Any]

    def __init__(self):
        self.idents = {}
        self.modules = {}

    def to_dict(self):
        return {
            "idents": self.idents,
            "modules": {k: v.to_dict() for k, v in self.modules.items()},
        }

    def insert(self, path: list[str], character: str) -> None:
        l = path.__len__()
        assert l > 0
        head = path[0]
        if l == 1:
            self.idents[head] = character
        else:
            if self.modules.get(head) is None:
                self.modules[head] = ModDict()
            assert self.modules.get(head) is not None
            self.modules[head].insert(path[1:], character)

    def patch(self, patch: "ModDict") -> None:
        for k, v in patch.idents.items():
            self.idents[k] = v
        for k, v in patch.modules.items():
            if self.modules.get(k) is None:
                self.modules[k] = ModDict()
            self.modules[k].patch(v)


def load_mod_dict(json: dict[str, Any]) -> ModDict:
    idents = json["idents"]
    modules = json["modules"]
    d = ModDict()
    for k, v in idents.items():
        d.idents[k] = v
    for k, v in modules.items():
        d.modules[k] = load_mod_dict(v)
    return d


path = "./dictionary.txt"
patch_path = "./dictionary_patch.json"

with open(path, "r", encoding="utf-8") as f:
    lines = f.readlines()
    d = ModDict()
    len = lines.__len__()
    for i in range(len // 4):
        name = lines[i * 4].strip()
        code_point = int(lines[i * 4 + 1].strip(), base=10)
        character = lines[i * 4 + 2].strip()
        command = lines[i * 4 + 3].strip()
        d.insert(name.split("."), character)

    with open(patch_path, "r", encoding="utf-8") as f:
        source = f.read()
        as_dict = json.loads(source)
        patch: ModDict = ModDict()
        patch = load_mod_dict(as_dict)

        d.patch(patch)

    # into json
    with open("./dictionary.json", "w", encoding="utf-8") as f:
        json.dump(d.to_dict(), f, indent=4, ensure_ascii=False)
