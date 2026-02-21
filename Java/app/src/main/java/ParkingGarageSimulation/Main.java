package ParkingGarageSimulation;

import java.util.Scanner;
import java.util.concurrent.Semaphore;

class Car implements Runnable {
	private Semaphore parkingSlots;
	private int carNumber;

	Car(Semaphore parkingSlots, int carNumber) {
		this.parkingSlots = parkingSlots;
		this.carNumber = carNumber;
	}

	@Override
	public void run() {
		System.out.printf("Car #%d has arrived.\n", carNumber);

		try {
			parkingSlots.acquire();
			System.out.println("Car #" + carNumber + " has parked.");
			Thread.sleep(2000);
			System.out.println("Car #" + carNumber + " is leaving.");
			parkingSlots.release();
		} catch (InterruptedException e) {
			e.printStackTrace();
		}
	}
}

public class Main {
	public static void main(String[] args) {
		Scanner scanner = new Scanner(System.in);

		System.out.print("Enter the number of parking slots: ");
		int totalParkingSlots = scanner.nextInt();

		System.out.print("Enter the number of cars: ");
		int totalCars = scanner.nextInt();

		Semaphore parkingSlots = new Semaphore(totalParkingSlots);
		Thread[] carThreads = new Thread[totalCars];

		for (int i = 1; i <= totalCars; i++) {
			carThreads[i - 1] = new Thread(new Car(parkingSlots, i));
			carThreads[i - 1].start();
		}

		for (Thread carThread : carThreads) {
			try {
				carThread.join();
			} catch (InterruptedException e) {
				e.printStackTrace();
			}
		}

		System.out.println("Simulation Ended.");
		scanner.close();
	}
}
