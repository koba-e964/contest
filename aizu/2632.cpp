#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;
typedef pair<int, int> PI;

const int DEBUG = 0;

int main(void){
  int h, w, n;
  cin >> h >> w >> n;
  vector<PI> rem(n);
  REP(i, 0, n) {
    int u, v;
    cin >> u >> v;
    v--;
    rem[i] = PI(u, v);
    assert ((u + v) % 2 == 1);
  }
  assert (w % 2 == 0);
  sort(rem.begin(), rem.end());
  VI ary(w);
  REP(i, 0, w / 2) { ary[i] = i * 2; }
  REP(i, 0, w / 2) { ary[w / 2 + i] = w - 1 - 2 * i; }
  REP(i, 0, n) {
    PI t = rem[i];
    int bias = t.first / 2;
    int flipped = t.first % 2;
    int v = t.second;
    int mapped_v = w - 1 - v / 2;
    if (flipped) {
      mapped_v = v / 2;
    }
    int x = mapped_v;
    int y = flipped ? w - 1 - x : w - x;
    assert (0 <= y && y < w);
    x = (x + w - bias % w) % w;
    y = (y + w - bias % w) % w;
    if (DEBUG) { cerr << "x=" << x << ", y=" << y << endl; }
    swap(ary[x], ary[y]);
    if (DEBUG) {
      cerr << "step " << t.first << ":";
      REP(i, 0, w) {
	cerr << " " << ary[i];
      }
      cerr << endl;
    }
  }
  VI inv(w, -100);
  REP(i, 0, w) {
    int bias = (h / 2) % w;
    int flipped = h % 2;
    int mapped_i;
    if (i % 2 == 1) {
      mapped_i = w - 1 - i / 2;
    } else {
      mapped_i = i / 2;
    }
    if (flipped) {
      mapped_i = w - 1 - mapped_i;
    }
    int x = (mapped_i + w - bias % w) % w;
    inv[ary[x]] = i;
    if (DEBUG) {
      cerr << i << " ==> " << x << "(" << ary[x] << ")" << endl;
    }
  }
  REP(i, 0, w) {
    cout << inv[i] + 1 << endl;
  }
}
