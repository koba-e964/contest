#include <algorithm>
#include <bitset>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

int main(void){
  int n;
  while ((cin >> n) && n) {
    vector<bitset<100> > bit(n);
    REP(i, 0, n) {
      bitset<100> sol;
      int k;
      cin >> k;
      REP(j, 0, k) {
	int t;
	cin >> t;
	t--;
	sol[t] = true;
      }
      bit[i] = sol;
    }
    int k;
    cin >> k;
    bitset<100> leakage;
    REP(i, 0, k) {
      int t;
      cin >> t;
      t--;
      leakage[t] = true;
    }
    VI cand;
    REP(i, 0, n) {
      if ((bit[i] & leakage) == leakage) {
	cand.push_back(i);
      }
    }
    if (cand.size() != 1) {
      cout << -1 << endl;
    } else {
      cout << cand[0] + 1 << endl;
    }
      
  }
}
