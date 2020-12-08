const fs = require("fs")

const BAG = "shiny gold"

const input = fs
  .readFileSync("input.txt", "utf-8")
  .split("\n")
  .reduce((acc, rule) => {
    const [parent, children] = rule.replace(".", "").split(" bags contain ")
    const parsedChildren = children.split(", ").reduce((acc, child) => {
      const [num, color1, color2] = child.split(" ")
      if (num !== "no") {
        acc[color1 + " " + color2] = Number(num)
      }
      return acc
    }, {})
    acc[parent] = parsedChildren
    return acc
  }, {})

function getRequiredBagAmount(rules, bag) {
  return Object.entries(rules[bag]).reduce((count, [child, childCount]) => {
    console.log({ [child]: childCount })
    count += childCount * getRequiredBagAmount(rules, child) + childCount
    return count
  }, 0)
}

console.log(getRequiredBagAmount(input, BAG))
