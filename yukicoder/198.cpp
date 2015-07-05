#include <iostream>
#include <cmath>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

ll b;
ll c[11];
int n;

const ll inf = 1LL << 60;

ll solve(ll p) {
  ll dif = 0, sum = 0;
  REP(i, 0, n) {
    sum += abs(c[i] - p);
    dif += p - c[i];
  }
  return dif > b ? inf : sum;
}

const ll W = 4e4;

int main(void){
  cin >> b >> n;
  REP(i, 0, n) {
    cin >> c[i];
  }
  int mini = 0;
  ll mi = solve(0);
  REP(i, 0, W) {
    ll s = solve(i * W);
    if (mi > s) {
      mini = i;
      mi = s;
    }
  }
  REP(i, max(0LL, mini * W - W), mini * W + W + 1) {
    ll s = solve(i);
    if (mi > s) {
      mi = s;
    }
  }
  cout << mi << endl;
}
