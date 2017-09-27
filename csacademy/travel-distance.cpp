#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
    int x = 0, y = 0;
    string s;
    cin >> s;
    REP(i, 0, s.length()) {
        if (s[i] == 'W') {
            x -= 1;
        }
        if (s[i] == 'E') {
            x += 1;
        }
        if (s[i] == 'N') {
            y += 1;
        }
        if (s[i] == 'S') {
            y -= 1;
        }
    }
    cout << abs(x) + abs(y) << endl;
}
