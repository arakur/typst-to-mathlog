from typing import Any
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


# main

default_path = "./dictionary_unicode.json"
patch_path = "./dictionary_patch.json"
out_path = "./dictionary.json"

with open(default_path, "r", encoding="utf-8") as f:
    source = f.read()
    default_dict = json.loads(source)
    d = load_mod_dict(default_dict)

    with open(patch_path, "r", encoding="utf-8") as f:
        source = f.read()
        as_dict = json.loads(source)
        patch = load_mod_dict(as_dict)

        d.patch(patch)

    # into json
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(d.to_dict(), f, indent=4, ensure_ascii=False)
