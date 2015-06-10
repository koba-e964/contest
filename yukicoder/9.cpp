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

const int N = 1501;
int a[N], b[N];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
  }
  int mi = 1000000;
  REP(i, 0, n) {
    typedef pair<PI, int> qtt;
    priority_queue<qtt, vector<qtt>, greater<qtt> > que;
    REP(j, 0, n) {
      que.push(qtt(PI(a[j], 0), j));
    }
    REP(j, 0, n) {
      qtt p = que.top(); que.pop();
      qtt newp = qtt(PI(p.first.first + b[(i + j) % n] / 2, p.first.second + 1),
		     p.second);
      que.push(newp);
    }
    int ma = 0;
    while (! que.empty()) {
      qtt p = que.top(); que.pop();
      ma = max(ma, p.first.second);
    }
    mi = min(mi, ma);
  }
  cout << mi << endl;
}
