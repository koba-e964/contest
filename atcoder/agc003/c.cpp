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

VI norm(const VI &a) {
  int n = a.size();
  vector<PI> b(n);
  REP(i, 0, n) {
    b[i] = PI(a[i], i);
  }
  sort(b.begin(), b.end());
  VI ret(n);
  REP(i, 0, n) {
    ret[b[i].second] = i;
  }
  return ret;
}



int main(void){
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  a = norm(a);
  int cnt = 0;
  REP(i, 0, (n + 1) / 2) {
    if (a[2 * i] % 2 != 0) {
      cnt++;
    }
  }
  cout << cnt << endl;
}
