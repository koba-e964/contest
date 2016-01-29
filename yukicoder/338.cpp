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
	int a, b;
    cin >> a >> b;
    REP(n, 1, 201) {
        REP(x, 0, n + 1) {
        int ta = (200 * x + n) / n / 2;
        int tb = (200 * (n - x) + n) / n / 2;
        if (ta == a && tb == b) {
            cout << n << endl;
            return 0;
        }
        }
    }
    assert(0);
}
