#include <algorithm>
#include <cassert>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int inf = 1.1e9;

int f(int d, int h1, int h2, int h3) {
	assert (h1 != h3);
	if (h1 < h3) {
		swap(h1, h3);
	}
	if (h3 < 0) { return -1; }
	if (h2 > h1 || h2 < h3) {
		return 0;
	}
  int sub1 = h3 == 0 ? inf : (h2 - h3 + 1 + d - 1) / d;
	int sub2 = (h1 - h2 + d) / d;
	if (max(h1 - sub2 * d, 0) == h3) { sub2 = h3 == 0 ? inf : (sub2 + 1); }
	sub1 = min(sub1, sub2);
	return sub1 == inf ? -1 : sub1;
}


int main(void){
	int d, h1, h2, h3;
	cin >> d >> h1 >> h2 >> h3;
	if (d == 0) {
		cout << (h1 != h3 && ((h1 < h2 && h2 > h3) || (h1 > h2 && h2 < h3)) ? 0: -1) << endl;
		return 0;
			
	}
	if (h1 == h3) {
		int r = f(d, h1, h2, h3 - d);
		cout << (r < 0 ? -1 : r + 1) << endl;
		return 0;
	}
	cout << f(d, h1, h2, h3) << endl;
}
