// variations of list
// var list = [10, 15, 3, 7]
// var list = [15, 4, 11, 3, 1, 12, 10, 7]

// more or less works and handles negative entries
// but is inefficient with large data sets
// probably O(n) time
function kSum1(kVal) {
	const list = [15, 27, 4, 11, 3, -10, 12, 7]
	let temp = 0
	for (i = 0; i < list.length; i++) {
		temp = kVal - list[i]
		if (list.includes(temp)) {
			return true
		}
	}
	return false
}

// slightly cleaner version of kSum1
function kSum2(kVal) {
	const list = [15, 4, 11, 3, -10, 12, 7, 27]
	for (i in list) {
		if (list.includes(kVal - list[i])) {
			return true
		}
	}
	return false
}

// use only array function methods?
// essentially the same as kSum1 and 2 but no for loop
// could use raw output from filter if the results of query are needed in other calculations
// or could overwrite old array (or save to new array) with data and check if a pair exists to return T / F
function kSum3(kVal) {
	let list = [15, 4, 11, 3, -10, 12, 7, 27]
	list = list.filter( a => list.includes(kVal-a) )
	return !!list.length
}

// use find to stop search upon finding first match
// fails on kSum4(6) as it can check for 3 against itself
function kSum4(kVal) {
	var list = [15, 4, 11, 3, -10, 12, 7, 27]
	return list.find( a => list.includes(kVal - a) ) !== undefined
}

// fix for checking against itself
function kSum5(kVal) {
	var list = [15, 4, 11, 3, -10, 12, 7, 27]
	let result = list.find((a, index) => list.slice(index + 1).includes(kVal - a)) !== undefined
	return result
}

alert(kSum5(6))