#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;


int w[11];
int b[11];

int dp[23];

int main(void){
  int nw, nb;
  cin >> nw;
  vector<int> v;
  REP(i, 0, nw) {
    cin >> w[i];
    v.push_back(w[i] * 2);
  }
  cin >> nb;
  REP(i, 0, nb) {
    cin >> b[i];
    v.push_back(b[i] * 2 + 1);
  }
  sort(v.begin(), v.end());
  REP(i, 0, v.size() + 1) {
    int ma = 1;
    REP(j, 0, i) {
      if ((v[i] - v[j]) % 2 && v[i] / 2 > v[j] / 2) {
	ma = max(dp[j] + 1, ma);
      }
    }
    dp[i] = ma;
  }
  int ma = 0;
  REP(i, 0, v.size() + 1) {
    ma = max(ma, dp[i]);
  }
  cout << ma << endl;
}
