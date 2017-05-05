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
  int n, m;
  cin >> n >> m;
  vector<pair<PI, int> > t;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    t.push_back(make_pair(PI(a, -b), i));
  }
  sort(t.begin(), t.end());
  int conn = 1;
  ll rem = 0;
  vector<PI> out(m, PI(-100, -1));
  vector<int> resp;
  REP(i, 0, m) {
    PI e = t[i].first;
    int orig_idx = t[i].second;
    if (e.second == -1) {
      // connect
      out[orig_idx] = PI(conn, conn + 1);
      rem += conn - 1;
      conn += 1;
    } else {
      if (rem <= 0) {
	cout << -1 << endl;
	return 0;
      }
      rem -= 1;
      resp.push_back(orig_idx);
    }
  }
  int pos = 0;
  REP(i, 1, n) {
    if (pos >= (int) resp.size()) { break; }
    REP(j, 0, i - 1) {
      if (pos >= (int) resp.size()) { break; }
      int idx = resp[pos];
      out[idx] = PI(i + 1, j + 1);
      pos += 1;
    }
  }
  REP(i, 0, m) {
    cout << out[i].first << " " << out[i].second << endl;
  }
}
