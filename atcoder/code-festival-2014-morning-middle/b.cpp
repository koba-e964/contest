#include <iostream>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef pair<int, int> PI;


const int W = 123456;
vector<PI> p[W];
int mn[W];

// Solved with the aid of http://kmjp.hatenablog.jp/entry/2014/11/10/1030
int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    int x, y;
    cin >> x >> y;
    p[x].push_back(PI(y, i));
  }

  REP(i, 0, m) {
    int x;
    cin >> x;
    mn[x] += 1;
  }
  priority_queue<PI, vector<PI>, greater<PI> > que;
  int cnt = 0;
  REP(h, 0, W) {
    REP(i, 0, p[h].size()) {
      que.push(p[h][i]);
    }
    int rem = mn[h];
    while (rem > 0 && not que.empty()) {
      PI t = que.top();
      que.pop();
      if (t.first >= h) {
	cnt += 1;
	rem--;
      }
    }
  }
  cout << cnt << endl;
}
