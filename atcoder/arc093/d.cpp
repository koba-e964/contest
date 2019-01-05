#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int a, b;
  cin >> a >> b;
  bool flip = false;
  if (a < b) {
    swap(a, b);
    flip = true;
  }
  int h = 100, w = 100;
  vector<VI> ans(h, VI(w, 0));
  int hf = w / 2;
  int pos = 0;
  REP(i, 0, h) {
    REP(j, 0, w) {
      ans[i][j] = i >= h / 2 ? 1 : 0;
    }
  }
  //DEBUGP(hf);
  //DEBUGP(dr);
  while (pos < a - 1) {
    ans[h / 2 + 1 + 2 * (pos / hf)][2 * (pos % hf)] = 0;
    pos++;
  }
  pos = 0;
  while (pos < b - 1) {
    ans[2 * (pos / hf)][2 * (pos % hf)] = 1;
    pos++;
  }

  if (flip) {
    REP(i, 0, h) {
      REP(j, 0, w) {
	ans[i][j] = 1 - ans[i][j];
      }
    }
  }

  cout << h << " " << w << endl;
  REP(i, 0, h) {
    REP(j, 0, w) {
      cout << (ans[i][j] ? '#' : '.'); 
    }
    cout << endl;
  }
}
