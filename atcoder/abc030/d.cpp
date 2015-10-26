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

int rem(const string &str, int q, int m) {
  if (str.length() >= 16) {
    int a = 0;
    REP(i, 0, str.length()) {
      a *= 10;
      a += str[i] - '0';
      a %= m;
    }
    a += m - (q % m);
    return (a % m) + q;
  }
  ll t;
  stringstream ss(str);
  ss >> t;
  if (t < q) { return t;}
  return (t - q) % m + q;
}

int main(void){
  int n, a;
  string k;
  cin >> n >> a >> k;
  a--;
  VI b(n);
  REP(i, 0, n) {
    cin >> b[i];
    b[i]--;
  }
  VI vis(n, -1);
  VI tbl(n);
  int cur = a;
  int pos = 0;
  int q, m;
  while (1) {
    if (vis[cur] >= 0) {
      q = vis[cur];
      m = pos - q;
      break;
    }
    tbl[pos] = cur;
    vis[cur] = pos;
    cur = b[cur];
    pos++;
  }
  cout << tbl[rem(k, q, m)] + 1 << endl;
}
