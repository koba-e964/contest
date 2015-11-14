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

ll c[7];
ll d[7];
bool ts() {
    int a = -1;
    REP(i, 0, 7) {
        if(d[i] == 0) {
            if (a == -1) a = i;
            else if (a == i - 1) {
                a = i;
            } else {
                return 0;
            }
        }
    }
    return 1;
}

bool dynamo() {
    return 0;
}

bool calc(void) {
    ll tot = 0;
    REP(i, 0, 7) tot += c[i];
    REP(i, 0, 7) {
        d[i] = tot - 2 * c[(i + 2) % 7] - 2 * c[(i + 4) % 7] - 2 * c[(i + 6) % 7];
        if (d[i] < 0) {
            return 0;
        }
    }
    REP(i, 0, 7) {
    }
    REP(i, 0, 7) {
        if (d[i] == 0) {
            return ts();
        }
    }
    return 1;
}

int main(void){
    REP(i, 0, 7) cin >> c[i];
    c[0]--;
    cout << (calc() ? "YES" : "NO") << endl;
}
