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
  int ma = 0;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  int p = 0;
  map<int, int> tbl;
  REP(i, 0, n) {
    if (tbl.count(a[i])) {
      int to = tbl[a[i]];
      while (p <= to) {
	tbl.erase(a[p]);
	p++;
      }
    }
    tbl.insert(PI(a[i], i));
    ma = max(ma, i - p + 1);
  }
  cout << ma << endl;
}
