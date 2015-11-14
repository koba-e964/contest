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
    string s;
    cin >>n >> s;
    int cnt= 0, pos = 0;
    while (pos < 2 * n - 1) {
        string sl = s.substr(pos, 2);
        if (sl == "01" || sl == "10") {
            pos += 2;
            continue;
        }
        cnt++;
        pos++;
    }
    cnt += 2 * n - pos;
    cout << cnt / 2 << endl;
}
