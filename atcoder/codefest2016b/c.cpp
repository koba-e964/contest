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
  int w, h;
  cin >> w >> h;
  VL p(w);
  VL q(h);
  typedef pair<ll, int> PLI;
  vector<PLI> que(w + h);
  REP(i, 0, w) {
    cin >> p[i];
    que[i] = PLI(p[i], i);
  }
  REP(i, 0, h) {
    cin >> q[i];
    que[w + i] = PLI(q[i], w + i);
  }
  sort(que.begin(), que.end());
  ll sum = 0;
  int cw = 0;
  int ch = 0;
  REP(i, 0, que.size()) {
    PLI cur = que[i];
    if (cur.second >= w) {
      sum += cur.first * (w + 1 - cw);
      ch++;
    } else {
      sum += cur.first * (h + 1 - ch);
      cw++;
    }
  }
  cout << sum << endl;
}
