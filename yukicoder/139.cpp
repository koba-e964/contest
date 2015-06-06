#include <algorithm>
#include <bitset>
#include <cassert>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  int n,l;
  cin >> n >> l;
  ll cur = 0;
  ll pos = 0;
  REP(i, 0, n) {
    int x, w, t;
    cin >> x >> w >> t;
    cur += x - pos;
    pos = x;
    ll stage = cur % (2 * t);
    if (stage > t - w) {
      cur += 2 * t - stage + w;
    } else {
      cur += w;
    }
    pos += w;
  }
  cout << cur + l - pos << endl;
}
