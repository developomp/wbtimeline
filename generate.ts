import markdownIt from "https://esm.sh/markdown-it"
import fm from "https://esm.sh/front-matter"

// data to be saved as data.json
let data: any[] = []

const md = markdownIt()

function recursiveParse(path: string): void {
	const stats = Deno.lstatSync(path)

	if (stats.isDirectory) {
		// call this function for every files and directories in it
		for (const childPath of Deno.readDirSync(path)) {
			recursiveParse(`${path}/${childPath.name}`)
		}
	} else if (stats.isFile) {
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

	let date = filePathArr[filePathArr.length - 1].slice(0, 10) // remove .md

	/**
	 * Checks
	 */

	if (date.length != 10) throw new Error(`Invalid file name: ${filePath}`)

	/**
	 * Parse markdown
	 */

	const parsedMarkdown = fm(Deno.readTextFileSync(filePath))

	/**
	 * Append data
	 */

	data.push({
		date: date,
		category: filePathArr[filePathArr.length - 3],
		subcategory: filePathArr[filePathArr.length - 2],
		description: md.render(parsedMarkdown.body),
		...(parsedMarkdown.attributes as any),
	})
}

function postProcess() {
	// sort data by date

	data = data.sort(
		(a: any, b: any) =>
			new Date(a.date as string).getTime() -
			new Date(b.date as string).getTime()
	)

	// format date
	data.map((entry) => {
		entry.date = new Date(entry.date).toLocaleString("default", {
			month: "short",
			day: "numeric",
			year: "numeric",
		})
		return entry
	})
}

function main() {
	recursiveParse("./data")
	postProcess()

	// save to file
	Deno.writeTextFileSync("./static/data.json", JSON.stringify(data))
}

main()
