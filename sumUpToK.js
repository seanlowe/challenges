// variations of list
// var list = [10, 15, 3, 7]
// var list = [15, 4, 11, 3, 1, 12, 10, 7]

// more or less works and handles negative entries
// but is inefficient with large data sets
// probably O(n) time
function kSum1(kVal) {
	var list = [15, 27, 4, 11, 3, -10, 12, 7]
	var temp = 0
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
	var list = [15, 4, 11, 3, -10, 12, 7, 27]
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
	var list = [15, 4, 11, 3, -10, 12, 7, 27]
	// console.log(list.filter( a => list.includes(kVal-a) ))
	list = list.filter( a => list.includes(kVal-a) )
	return (list.length % 2 == 0) ? true : false // allows finding multiple pairs
}

// alert(kSum1(17))
// alert(kSum2(17))
alert(kSum3(19))