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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  int n;
  cin >> n;
  VI l(n), r(n);
  int lt = 0, rt = 0;
  REP(i, 0, n) {
    cin >> l[i] >> r[i];
    lt += l[i];
    rt += r[i];
  }
  int ma = abs(lt - rt);
  int maxi = -1;
  REP(i, 0, n) {
    int ltn = lt + r[i] - l[i];
    int rtn = rt + l[i] - r[i];
    int d = abs(ltn - rtn);
    if (ma < d) {
      ma = d;
      maxi = i;
    }
  }
  cout << maxi + 1 << endl;
}
