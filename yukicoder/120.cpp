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
const double EPS=1e-9;



int main(void){
  int t;
  cin >> t;
  REP(loop_var, 0, t) {
    int n;
    cin >> n;
    map<int, int> m;
    REP(i, 0, n) {
      int l;
      cin >> l;
      if (m.count(l) == 0) {
	m[l] = 0;
      }
      ++m[l];
    }
    priority_queue<int, vector<int> > que;
    for (map<int, int>::iterator it = m.begin(); it != m.end(); ++it) {
      que.push(it->second);
    }
    que.push(0);
    que.push(0);
    que.push(0);
    int cnt = 0;
    while (1) {
      int a0 = que.top(); que.pop();
      int a1 = que.top(); que.pop();
      int a2 = que.top(); que.pop();
      if (a2 <= 0) {
	break;
      }
      cnt++;
      que.push(a0 - 1);
      que.push(a1 - 1);
      que.push(a2 - 1);
    }
    cout << cnt << endl;
  }
}
