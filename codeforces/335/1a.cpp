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
  VI b(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
    b[a[i]] = i;
  }
  int ma = 0;
  int cu = 0, st = 0;
  REP(i, 0, n) {
    if (b[i] > cu) {
      cu = b[i];
      continue;
    }
    ma = max(ma, i - st);
    st = i;
    cu = b[i];
  }
  ma = max(ma, n - st);
  cout << (n - ma) << endl;
}
