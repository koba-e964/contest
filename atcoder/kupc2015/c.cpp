#include <algorithm>
#include <string>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

typedef vector<int> VI;

int main(void){
  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    const int inf = 5e7;
    vector<VI> dist(n, VI(n));
    REP(i, 0, n) {
      REP(j, 0, n) {
	int tmp;
	cin >> tmp;
	dist[i][j] = tmp == -1 ? inf : tmp;
      }
    }
    vector<VI> dist2(dist);
    REP(i, 0, n) {
      dist2[i][i] = 0;
    }
    REP(k, 0, n) {
      REP(i, 0, n) {
	REP(j, 0, n) {
	  dist2[i][j] = min(dist2[i][j], dist2[i][k] + dist2[k][j]);
	}
      }
    }
    bool ok = true;
    REP(i, 0, n) {
      REP(j, 0, n) {
	ok &= dist[i][j] == dist2[i][j];
      }
    }
    cout << (ok ? "YES" : "NO") << endl;
  }
}
