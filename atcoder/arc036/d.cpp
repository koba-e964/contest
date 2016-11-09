#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int N = 100100;
int evenodd[N] = {};
// QuickFind structure
// Reference: https://topcoder.g.hatena.ne.jp/iwiwi/20131226/1388062106
vector<int> cs[N];
int root[N];
int cont[N];

void merge(int x, int y) {
  int rx = root[x];
  int ry = root[y];
  if (rx == ry) {
    return;
  }
  if (cs[rx].size() < cs[ry].size()) {
    swap(rx, ry);
  }
  for (vector<int>::iterator it = cs[ry].begin(); it != cs[ry].end(); ++it) {
    int v = *it;
    cs[rx].push_back(v);
    root[v] = rx;
  }
  cs[ry].clear();
  if (cont[ry]) {
    cont[rx] = 1;
    cont[ry] = 0;
  }
}

int main(void){
  int n, q;
  cin >> n >> q;
  REP(i, 0, n) {
    cs[i].push_back(i);
    root[i] = i;
  }
  REP(i, 0, q) {
    int w, x, y, z;
    cin >> w >> x >> y >> z;
    x--, y--;
    z %= 2;
    if (w == 1) { // create a road
      if (root[x] == root[y]) {
	if ((evenodd[x] + evenodd[y] + z) % 2) {
	  // contradiction!!
	  cont[root[x]] = 1;
	}
      } else {
	int rx = root[x], ry = root[y];
	if (cs[rx].size() < cs[ry].size()) {
	  swap(rx, ry);
	  swap(x, y);
	}
	if ((evenodd[x] + evenodd[y] + z) % 2) {
	  // minority should be converted
	  REP(j, 0, cs[ry].size()) {
	    int &r = evenodd[cs[ry][j]];
	    r = 1 - r;
	  }
	}
	merge(x, y);
      }
    } else {
      if (root[x] == root[y]) {
	int rx = root[x];
	bool able = cont[rx] || evenodd[x] == evenodd[y];
	cout << (able ? "YES" : "NO") << endl;
      } else {
	cout << "NO" << endl;
      }
    }
    if (0) {
      REP(i, 0, n) {
	cerr << "[" << i << "]";
	REP(j, 0, cs[i].size()) {
	  cerr << cs[i][j] << " ";
	}
	if (cont[i]) {
	  cerr << "cont ";
	}
	if (root[i] != i) {
	  cerr << "root=" << root[i] << " ";
	}
	cerr << "eo=" << evenodd[i];
	cerr << endl;
      }
      cerr << endl;
    }
  }
}
