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
    string s;
    cin >> s;
    int n = s.length();
    int ex = 0;
    REP(i, 0, n) {
        if (s[i] == '!') ex++;
    }
    int mi = 0;
    REP(i, 0, n) {
            if (s[i] == '-') { mi++;}
            else break;
    }
    if (ex >= 3) ex = ((ex - 1) % 2) + 1;
    mi %= 2;
    assert(mi <= 1);
    assert(ex <= 2);
    REP(i, 0, mi) cout << "-";
    REP(i, 0, ex) cout << "!";
    cout << endl;
}
