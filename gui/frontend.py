import sys
import signal
from PyQt5.QtCore import *
from PyQt5.QtWidgets import *

app = QApplication([])

#label = QLabel('Hello World! Here is a RUST function output: ' + subtractesizer.sum_as_string(10, 90))
#label.show()

class MainWindow(QWidget):
    #volSlide=pyqtSignal(int)
    def __init__(self,parent=None):
        #UI
        QWidget.__init__(self,parent)

        title = QLabel()
        title.setText('Subtractesizer')

        self.label = QLabel()
        self.label.setText('default')

        #volume
        self.volume_dial = QDial(self)
        self.volume_dial.setMinimum(0)
        self.volume_dial.setMaximum(100)

        #qfactor 
        self.qfac_dial = QDial()
        self.qfac_dial.setMinimum(0)
        self.qfac_dial.setMaximum(1000)
        self.qfac_dial.setValue(5000)

        #connect signals and slots
        self.quit_button = QPushButton(self.tr('&Quit'))
        self.quit_button.clicked.connect(self.on_quit_clicked)
        self.volume_dial.valueChanged.connect(self.on_volume_changed)

        #layout 
        layout = QVBoxLayout(self)
        layout.addWidget(self.volume_dial)
        layout.addWidget(self.qfac_dial)
        layout.addWidget(self.label)

        #show ui
        self.volume_dial.show()
        self.volume_dial.show()
        title.show()

    @pyqtSlot(int)
    def on_volume_changed(self, volume):
        self.label.setText("".format(volume))
        
    @pyqtSlot()
    def on_quit_clicked(self):
        self.close()

def main():

    main_window = MainWindow()
    main_window.show()

    sys.exit(app.exec_())

if __name__ == '__main__':
    main()
    