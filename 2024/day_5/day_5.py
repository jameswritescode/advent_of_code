import math
import sys

from collections import defaultdict

type Rules = dict[int, list[int]]

class Validator:
    def __init__(self, rules: Rules, updates: list[list[int]]):
        self.rules = rules
        self.updates = updates

    @classmethod
    def parse(cls, path):
        with open(path) as file:
            content = file.read()

        content = content.rstrip().split('\n\n')
        rules, updates = [section.split('\n') for section in content]
        rules = [cls.__parse_input_list(rule, '|') for rule in rules]
        updates = [cls.__parse_input_list(update, ',') for update in updates]

        return cls(cls.__parse_rules(rules), updates)

    @classmethod
    def __parse_input_list(cls, input, split_on):
        return list(map(int, input.split(split_on)))

    @classmethod
    def __parse_rules(cls, rules) -> Rules:
        mapping = defaultdict(list)

        for rule in rules:
            mapping[rule[0]].append(rule[1])

        return mapping


    def valid_updates(self):
        valid_updates = []

        for update in self.updates:
            if self.__valid_update(update):
                valid_updates.append(update)

        return valid_updates

    def __valid_update(self, pages: list[int]):
        seen = []

        for page in pages:
            subsequent_pages = self.rules[page]
            subsequent_page_seen = any(sp in seen for sp in subsequent_pages)

            if subsequent_page_seen:
                return False

            seen.append(page)

        return True

def part1():
    valid_updates = Validator.parse(sys.argv[1]).valid_updates()
    middle = lambda update: update[math.floor(len(update) / 2)]
    print(sum([middle(update) for update in valid_updates]))

part1()
