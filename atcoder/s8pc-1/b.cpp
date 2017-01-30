#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int h, w, n;
const int N = 20;
int x[N], y[N];

bool check(int a, int b) {
  int cnt = 0;
  REP(i, 0, n) {
    int res = a * x[i] + b * y[i];
    if (res == 0) {
      return false;
    }
    cnt += res > 0 ? 1 : 0;
  }
  return 2 * cnt == n;
}

int main(void){
  cin >> h >> w >> n;
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  vector<PI> res;
  REP(i, 1, w) {
    if (check(h, -i)) {
      res.push_back(PI(i, h));
    }
  }
  REP(i, 1, h + 1) {
    if (check(i, -w)) {
      res.push_back(PI(w, i));
    }
  }
  if (res.size() > 0) {
    REP(i, 0, res.size()) {
      PI p = res[i];
      cout << "(" << p.first << "," << p.second << ")\n";
    }
  } else {
    cout << -1 << endl;
  }
}
