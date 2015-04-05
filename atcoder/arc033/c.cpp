#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;
/**
 * holds the array (1.. size)
 *
 */
template <class T> class BIT{
private:
	int n;
	T* ary;
public:
	BIT(int n) {
		if(n<=0)
			assert(0);
		while(n != (n & (-n))) {
			n += n & (-n);
		}
		this->n = n;
		ary = new T[n+1];
		for(int i=0; i<=n; i++) {
			ary[i] = 0;
		}
	}
	~BIT(void) {
		delete [] ary;
	}
	/**
	 * gets the sum in [1 .. ind]
	 * @param ind
	 * @return sum
	 */
	T accum(int ind) {
		T sum=0;
		while(ind>0) {
			sum += ary[ind];
			ind &= ind - 1;
		}
		return sum;
	}
	/**
	 * performs data[ind] += val;
	 */
	void add(int ind, T val) {
		if(ind <= 0) {
			assert(0);
		}
		while(ind <= n) {
			ary[ind] += val;
			ind += ind & (-ind);
		}
	}
	T size() {
		return ary[n];
	}
};



int main(void){
	int q;
	cin >> q;
	BIT<int> bit(200010);
	REP(i, 0, q) {
		int t,x;
		cin >> t >> x;
		if (t == 1) { //add
			bit.add(x, 1);
		}
		else {
			// calc minimum e s.t. bit.accum(e) >= x
			int a = 0, b = 200001;
			while (b - a >= 2) {
				int m = (a + b) / 2;
				if (bit.accum(m) >= x) {
					b = m;
				} else {
					a = m;
				}
			}
			cout << b << endl;
			bit.add(b, -1);
		}
	}
}
