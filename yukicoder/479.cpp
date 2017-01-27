#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;
typedef pair<int, int> PI;

const int N = 100100;
VI link[N];

int main(void){
  int n, m;
  cin >> n >> m;
  vector<PI> e;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    e.push_back(PI(a, b));
    link[a].push_back(e.size() - 1);
    link[b].push_back(e.size() - 1);
  }
  VI ans(n);
  for (int i = n - 1; i >= 0; --i) {
    // check if Vertex i is necessary or not
    bool usei = false;
    for (auto ei: link[i]) {
      if (e[ei].first == i) {
	// cannot avoid using i
        usei = true;
      }
    }
    if (not usei) {
      ans[i] = 0;
      continue;
    }
    ans[i] = 1;
    // eliminate all connecting edges
    for (auto ei: link[i]) {
      e[ei].first = -1;
    }
  }
  int ma = 0;
  REP(i, 0, n) {
    if (ans[i]) ma = i;
  }
  for (int i = ma; i >= 0; --i) {
    cout << ans[i];
  }
  cout << endl;
}
