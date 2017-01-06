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


// This solution is made after the author read the editorial. 
int main(void){
  int n;
  cin >> n;
  vector<double> l(n);
  REP(i, 0, n) {
    cin >> l[i];
  }
  int q;
  cin >> q;
  VI k(q);
  REP(i, 0, q) {
    cin >> k[i];
  }
  const int K = 500001;
  typedef pair<double, int> PDI;
  priority_queue<PDI> que;
  REP(i, 0, n) {
    que.push(PDI(l[i], i));
  }
  VI use(n, 0);
  double mi = 1e10;
  vector<double> res(K);
  REP(i, 1, K) {
    PDI t = que.top(); que.pop();
    int idx = t.second;
    use[idx]++;
    mi = min(mi, l[idx] / use[idx]);
    res[i] = mi;
    que.push(PDI(l[idx] / (use[idx] + 1), idx));
  }
  REP(i, 0, q) {
    printf("%.15f\n", res[k[i]]);
  }    
}
