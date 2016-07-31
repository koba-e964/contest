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
  int n, m;
  cin >> n >> m;
  VI sticky(n);
  VI num(n);
  sticky[0] = 1;
  REP(i, 0, n) {
    num[i] = 1;
  }
  REP(i, 0, m) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    if (sticky[x]) {
      sticky[y] = 1;
    }
    num[x]--;
    num[y]++;
    if (num[x] == 0) {
      sticky[x] = 0;
    }
  }
  cout << count(sticky.begin(), sticky.end(), 1) << endl;
}
