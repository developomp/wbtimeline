import markdownIt from "markdown-it"
import fm from "front-matter"
import fs from "fs"

// data to be saved as data.json
const data: any = {}

const md = markdownIt()

function recursiveParse(path: string): void {
	const stats = fs.lstatSync(path)

	if (stats.isDirectory()) {
		// call this function for every files and directories in it
		fs.readdirSync(path).map((childPath) => {
			recursiveParse(`${path}/${childPath}`)
		})
	} else if (stats.isFile()) {
		parseFile(path)
	}
}

function parseFile(filePath: string): void {
	// stop if it is not a markdown file
	if (!filePath.endsWith(".md")) {
		console.warn(`Ignoring non markdown file at: ${filePath}`)
		return
	}

	const filePathArr = filePath.split("/")

	const date = filePathArr[filePathArr.length - 1].slice(0, 10) // remove .md
	const category = filePathArr[filePathArr.length - 3]
	const subcategory = filePathArr[filePathArr.length - 2]

	/**
	 * Checks
	 */

	if (date.length != 10) throw new Error(`Invalid file name: ${filePath}`)

	/**
	 * Parse markdown
	 */

	const parsedMarkdown = fm(fs.readFileSync(filePath, "utf8"))

	if (!data[category]) data[category] = {}
	if (!data[category][subcategory]) data[category][subcategory] = {}

	data[category][subcategory][date] = {
		description: md.render(parsedMarkdown.body),
		...(parsedMarkdown.attributes as any),
	}
}

function main() {
	recursiveParse("./data")

	// save to file
	fs.writeFileSync("./static/data.json", JSON.stringify(data))
}

main()
