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

string days[7] = {"Monday", "Tuesday", "Wednesday",
"Thursday", "Friday", "Saturday", "Sunday"};

int len[12] = {31,28,31,30,31,30,31,31,30,31,30,31};

int main(void) {
    string d;
    cin >> d;
    int i = find(days, days + 7, d) - days;
    int cnt = 0;
    REP(j, 0, 12) {
        if ((i + 12) % 7 == 4) {
            cnt += 1;
        }
        i += len[j];
    }
    cout << cnt << endl;
}
