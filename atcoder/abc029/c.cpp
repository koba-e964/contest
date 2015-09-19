#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
	int n;
	cin >> n;
	int lim = pow(3, n);
	REP(i, 0, lim) {
		char s[9] = {};
		int c = i;
		REP(j, 0, n) {
			s[n - 1 - j] = c % 3 + 'a';
			c /= 3;
		}
		cout << s << endl;
	}
}
