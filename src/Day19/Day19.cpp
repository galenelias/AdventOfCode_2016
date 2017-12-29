#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <map>
#include <algorithm>
#include <forward_list>

using namespace std;

size_t solve(size_t initial_size)
{
	list<size_t> elves;
	for (size_t i = initial_size; i != 0; --i)
		elves.push_front(i);

	auto iter1 = elves.begin();
	auto iter2 = elves.begin();

	// Move iter2 half-way around the list
	for (size_t i = 0; i < initial_size / 2; ++i)
		++iter2;

	while (iter1 != iter2)
	{
		// Elf 1 steals from elf  2.  Move to next person across the circle which depends on the parity of the size of the circle
		iter2 = elves.erase(iter2);
		if (iter2 == end(elves))
			iter2 = begin(elves);

		if ((elves.size() % 2) == 0)
		{
			iter2++;
			if (iter2 == end(elves))
				iter2 = begin(elves);
		}

		++iter1;
		if (iter1 == end(elves))
			iter1 = begin(elves);

	}

	return *iter1;
}

size_t solve2(size_t input)
{
	size_t powthree = 1;
	while (powthree * 3 < input)
		powthree *= 3;

	if ((input - powthree) / powthree >= 1)
		return input - powthree + (input - powthree - powthree);

	return input - powthree;
}



int main()
{
	size_t input;
	cin >> input;

	cout << "Part 2: " << solve(input) << endl;
	cout << "Part 2: " << solve2(input) << endl;

	return 0;
}

