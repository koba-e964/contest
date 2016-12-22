#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  int m;
  cin >> m;
  VL x(m), y(m);
  REP(i, 0, m) {
    cin >> x[i] >> y[i];
  }
  ll ox, oy;
  cout << "? 0 0" << endl;
  cin >> ox >> oy;
  ll kx, ky;
  cout << "? 1 0" << endl;
  cin >> kx >> ky;
  kx -= ox;
  ky -= oy;
  ll lx, ly;
  cout << "? 0 1" << endl;
  cin >> lx >> ly;
  lx -= ox;
  ly -= oy;
  cout << "!" << endl;
  REP(i, 0, m) {
    cout << ox + kx * x[i] + lx * y[i];
    cout << " ";
    cout << oy + ky * x[i] + ly * y[i];
    cout << endl;
  }
}
