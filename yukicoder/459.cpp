#include <cassert>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  int h, w, n;
  cin >> h >> w >> n;
  VI pile(w);
  REP(i, 0, h) {
    string s;
    cin >> s;
    assert (s.length() == w);
    REP(j, 0, w) {
      if (s[j] == '#') { pile[j]++; }
    }
  }
  VI c(n);
  vector<VI> blk(w - 2);
  REP(i, 0, n) {
    cin >> c[i];
    blk[c[i]].push_back(i);
  }
  vector<VI> ans(n, VI(3, 0));
  for (int i = w - 3; i >= 0; --i) {
    REP(j, 0, blk[i].size()) {
      int idx = blk[i][j];
      bool found = 0;
      for (int k = 2; k >= 0; --k) {
	if (pile[i + k] >= 1) {
	  ans[idx][k]++;
	  pile[i + k]--;
	  found = 1;
	  break;
	}
      }
      assert (found);
    }
  }
  REP(i, 0, w - 2) {
    REP(j, 0, blk[i].size()) {
      int idx = blk[i][j];
      REP(k, 0, 3) {
	int diff = min(pile[i + k], 3 - ans[idx][k]);
	pile[i + k] -= diff;
	ans[idx][k] += diff;
      }
    }
  }
  REP(i, 0, n) {
    REP(j, 0, 3) {
      REP(k, 0, 3) {
	cout << (ans[i][k] > j ? '#' : '.');
      }
      cout << endl;
    }
  }
}
