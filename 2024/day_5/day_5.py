import sys

from collections import defaultdict


class PageOrderer:
    def __init__(self, rules: dict[int, list[int]], updates: list[list[int]]):
        self.rules = rules
        self.updates = updates

    @classmethod
    def parse(cls, path):
        with open(path) as file:
            content = file.read()

        content = content.rstrip().split("\n\n")
        rules, updates = [section.split("\n") for section in content]

        return cls(cls.__parse_rules(rules), cls.__parse_section(updates, ","))

    @staticmethod
    def __parse_section(input, split_on):
        return [[int(i) for i in line.split(split_on)] for line in input]

    @classmethod
    def __parse_rules(cls, input):
        rules = cls.__parse_section(input, "|")
        mapping = defaultdict(list)

        for rule in rules:
            mapping[rule[0]].append(rule[1])

        return mapping

    def ensure_ordering(self):
        valid_updates = []
        repaired_updates = []

        for update in self.updates:
            (result, repair) = self.__order(update)

            if repair:
                repaired_updates.append(result)
            else:
                valid_updates.append(result)

        return valid_updates, repaired_updates

    def __order(self, pages):
        seen = []
        repaired = False

        for page in pages:
            subsequent_pages = self.rules[page]
            subsequent_pages_seen = [sp for sp in subsequent_pages if sp in seen]

            if any(subsequent_pages_seen):
                repaired = True
                num = next(p for p in seen if p in subsequent_pages_seen)
                seen.insert(seen.index(num), page)
            else:
                seen.append(page)

        return seen, repaired


def calc(updates):
    print(sum([update[len(update) // 2] for update in updates]))


validator = PageOrderer.parse(sys.argv[1])
result = validator.ensure_ordering()

calc(result[0])
calc(result[1])
