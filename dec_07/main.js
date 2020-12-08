const fs = require("fs")

const BAG = "shiny gold"

// cheat day...

function parseRule(rule) {
  const [parent, children] = rule.replace(".", "").split(" bags contain ")
  const parsedChildren = children.split(", ").reduce((acc, child) => {
    const [num, color1, color2] = child.split(" ")
    if (num !== "no") {
      acc[color1 + " " + color2] = num
    }
    return acc
  }, {})
  return [parent, parsedChildren]
}

const input = fs.readFileSync("input.txt", "utf-8").split("\n").map(parseRule)

function getPossibleBags(rules, bag, set = new Set()) {
  const addedBags = []

  for (const [parent, children] of rules) {
    if (bag in children) {
      set.add(bag)
      addedBags.push(parent)
    }
  }

  const values = addedBags.reduce((allBags, bag) => {
    return [...allBags, ...getPossibleBags(rules, bag)]
  }, addedBags)

  return [...new Set(values)]
}

console.log(getPossibleBags(input, BAG).length)
