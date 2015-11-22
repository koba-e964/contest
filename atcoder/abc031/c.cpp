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
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  int ma = -1e8;
  REP(i, 0, n) {
    int ma2 = -1e8, ma2b = -1, ma2e = -1;
    REP(j, 0, i) {
      int tota =0;
      for(int k = j + 1; k <= i; k += 2) {
	tota += a[k];
      }
      if (ma2 < tota) {
	ma2 = tota;
	ma2b = j;
	ma2e = i;
      }
    }
    REP(j, i + 1, n) {
      int tota =0;
      for(int k = i + 1; k <= j; k += 2) {
	tota += a[k];
      }
      if (ma2 < tota) {
	ma2 = tota;
	ma2b = i;
	ma2e = j;
      }
    }
    int tott = 0;
    for(int k = ma2b; k <= ma2e; k += 2) {
      tott += a[k];
    }
    ma = max(ma, tott);
  }
  cout << ma << endl;
}
